import hashlib

#Input the path of both the files
print ("Enter the path of first file.")
filePath1=input()
print ("Enter the path of second file.")
filePath2=input()

#Encode the strings 
filePath1=filePath1.encode('utf-8')
filePath2=filePath2.encode('utf-8')

#Finds its hash values
m1=hashlib.md5(filePath1)
m2=hashlib.md5(filePath2)

#Convert them into hexa decimal
a1=m1.hexdigest()
a2=m2.hexdigest()

#Print the values
print (a1)
print (a2+"\n")

if a1==a2:
	print ("Both are identical....No difference")
else:
	print ("They are different....You missed something")